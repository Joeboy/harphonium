import React, { useRef } from 'react';
import './Keyboard.css';

interface KeyboardProps {
  onNoteStart: (frequency: number) => void;
  onNoteStop: () => void;
  octaves: number;
  selectedKey: string;
  selectedScale: string;
  showNoteNames: boolean;
  transpose: number;
}

interface KeyData {
  frequency: number;
  note: string;
  isBlack?: boolean;
}

const Keyboard: React.FC<KeyboardProps> = ({ onNoteStart, onNoteStop, octaves, selectedKey, selectedScale, showNoteNames, transpose }) => {
  const containerRef = useRef<HTMLDivElement>(null);

  // Check if a note is in the selected scale
  const isNoteInScale = (note: string, key: string, scale: string): boolean => {
    if (scale === 'chromatic') return true;
    
    // Remove octave number from note (e.g., "C4" -> "C")
    const noteWithoutOctave = note.replace(/\d+$/, '');
    
    // Define scale intervals (semitones from root)
    const scaleIntervals: { [key: string]: number[] } = {
      'major': [0, 2, 4, 5, 7, 9, 11],
      'minor': [0, 2, 3, 5, 7, 8, 10],
      'major_pentatonic': [0, 2, 4, 7, 9],
      'minor_pentatonic': [0, 3, 5, 7, 10]
    };
    
    // Map note names to semitones
    const noteToSemitone: { [key: string]: number } = {
      'C': 0, 'C#': 1, 'D': 2, 'D#': 3, 'E': 4, 'F': 5,
      'F#': 6, 'G': 7, 'G#': 8, 'A': 9, 'A#': 10, 'B': 11
    };
    
    const intervals = scaleIntervals[scale];
    if (!intervals) return true; // Fallback to allow all notes
    
    const rootSemitone = noteToSemitone[key];
    const noteSemitone = noteToSemitone[noteWithoutOctave];
    
    if (rootSemitone === undefined || noteSemitone === undefined) return true;
    
    // Calculate the interval from the key to this note
    const interval = (noteSemitone - rootSemitone + 12) % 12;
    
    return intervals.includes(interval);
  };

  // Generate piano keys dynamically based on octaves setting
  const generateKeys = (numOctaves: number): KeyData[] => {
    // Base frequencies for one octave starting from C4
    const baseOctave = [
      { note: 'C', frequency: 261.63, isBlack: false },
      { note: 'C#', frequency: 277.18, isBlack: true },
      { note: 'D', frequency: 293.66, isBlack: false },
      { note: 'D#', frequency: 311.13, isBlack: true },
      { note: 'E', frequency: 329.63, isBlack: false },
      { note: 'F', frequency: 349.23, isBlack: false },
      { note: 'F#', frequency: 369.99, isBlack: true },
      { note: 'G', frequency: 392.0, isBlack: false },
      { note: 'G#', frequency: 415.3, isBlack: true },
      { note: 'A', frequency: 440.0, isBlack: false },
      { note: 'A#', frequency: 466.16, isBlack: true },
      { note: 'B', frequency: 493.88, isBlack: false },
    ];

    const keys: KeyData[] = [];
    const startOctave = 4; // Start from C4

    // Calculate total number of keys needed
    // For complete octaves (1, 2, 3), add 1 extra key to go from C to C
    // For fractional octaves (1.5, 2.5), use the fractional calculation
    const isCompleteOctave = numOctaves % 1 === 0;
    const totalKeys = isCompleteOctave 
      ? Math.floor(numOctaves) * 12 + 1  // Complete octaves: C to C (13, 25, 37 keys)
      : Math.ceil(numOctaves * 12);      // Fractional octaves: as before
    
    // Generate keys from high to low (C6 down to C4) for consistent ordering
    for (let i = totalKeys - 1; i >= 0; i--) {
      const octaveOffset = Math.floor(i / 12);
      const noteIndex = i % 12;
      const currentOctave = startOctave + octaveOffset;
      
      const baseNote = baseOctave[noteIndex];
      // Calculate frequency using the octave multiplier and transpose offset
      // Each octave doubles the frequency, and each semitone is 2^(1/12) ratio
      const frequency = baseNote.frequency * Math.pow(2, octaveOffset + transpose / 12);
      
      keys.push({
        frequency: frequency,
        note: `${baseNote.note}${currentOctave}`,
        isBlack: baseNote.isBlack
      });
    }

    return keys;
  };

  const keys = generateKeys(octaves);

  // Optimized touch handlers with minimal latency
  const createTouchHandlers = (freq: number, enabled: boolean) => {
    const handleStart = (e: React.TouchEvent | React.MouseEvent) => {
      if (!enabled) return; // Don't play if note is disabled
      
      e.preventDefault(); // Prevent default touch behaviors
      e.stopPropagation(); // Stop event bubbling

      // Use setTimeout with 0 delay to break out of React's batching for faster execution
      setTimeout(() => onNoteStart(freq), 0);
    };

    const handleEnd = (e: React.TouchEvent | React.MouseEvent) => {
      if (!enabled) return; // Don't process if note is disabled
      
      e.preventDefault();
      e.stopPropagation();
      setTimeout(() => onNoteStop(), 0);
    };

    return { handleStart, handleEnd };
  };

  return (
    <div className="keyboard-container" ref={containerRef}>
      <div className="keyboard">
        {keys.map((key) => {
          const inScale = isNoteInScale(key.note, selectedKey, selectedScale);
          const { handleStart, handleEnd } = createTouchHandlers(key.frequency, inScale);
          const dynamicStyle = {
            flex: key.isBlack ? '0.8' : '1', // Black keys take 80% of white key height
            minHeight: key.isBlack ? '12px' : '15px', // Minimum heights
          };
          
          return (
            <button
              key={key.note}
              className={`key ${key.isBlack ? 'black-key' : 'white-key'} ${!inScale ? 'disabled' : ''}`}
              style={dynamicStyle}
              onTouchStart={handleStart}
              onTouchEnd={handleEnd}
              onMouseDown={handleStart}
              onMouseUp={handleEnd}
              onMouseLeave={handleEnd}
              disabled={!inScale}
            >
              {showNoteNames && <div className="key-note">{key.note}</div>}
            </button>
          );
        })}
      </div>
    </div>
  );
};

export default Keyboard;

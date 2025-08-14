import React, { useRef } from 'react';
import './Keyboard.css';

interface KeyboardProps {
  onNoteStart: (frequency: number) => void;
  onNoteStop: () => void;
  octaves: number;
  selectedKey: string;
  selectedScale: string;
}

interface KeyData {
  frequency: number;
  note: string;
  isBlack?: boolean;
}

const Keyboard: React.FC<KeyboardProps> = ({ onNoteStart, onNoteStop, octaves, selectedKey, selectedScale }) => {
  const containerRef = useRef<HTMLDivElement>(null);

  // Define scale patterns (semitone intervals from root note)
  const scalePatterns = {
    chromatic: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], // All notes
    pentatonic: [0, 2, 4, 7, 9], // Major pentatonic pattern
  };

  // Note names in chromatic order
  const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];

  // Function to check if a note is in the selected scale
  const isNoteInScale = (noteName: string): boolean => {
    if (selectedScale === 'chromatic') return true;
    
    // Get the note without octave number
    const noteWithoutOctave = noteName.replace(/\d+$/, '');
    
    // Find the semitone offset from the selected key
    const keyIndex = noteNames.indexOf(selectedKey);
    const noteIndex = noteNames.indexOf(noteWithoutOctave);
    
    if (keyIndex === -1 || noteIndex === -1) return true; // Fallback to enabled
    
    // Calculate the interval from the key
    const interval = (noteIndex - keyIndex + 12) % 12;
    
    // Check if this interval is in the scale pattern
    const pattern = scalePatterns[selectedScale as keyof typeof scalePatterns] || scalePatterns.chromatic;
    return pattern.includes(interval);
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
      // Calculate frequency using the octave multiplier (each octave doubles the frequency)
      const frequency = baseNote.frequency * Math.pow(2, octaveOffset);
      
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
  const createTouchHandlers = (freq: number) => {
    const handleStart = (e: React.TouchEvent | React.MouseEvent) => {
      e.preventDefault(); // Prevent default touch behaviors
      e.stopPropagation(); // Stop event bubbling

      // Use setTimeout with 0 delay to break out of React's batching for faster execution
      setTimeout(() => onNoteStart(freq), 0);
    };

    const handleEnd = (e: React.TouchEvent | React.MouseEvent) => {
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
          const { handleStart, handleEnd } = createTouchHandlers(key.frequency);
          const inScale = isNoteInScale(key.note);
          const dynamicStyle = {
            flex: key.isBlack ? '0.8' : '1', // Black keys take 80% of white key height
            minHeight: key.isBlack ? '12px' : '15px', // Minimum heights
            opacity: inScale ? 1 : 0.3, // Dim disabled keys
          };
          
          return (
            <button
              key={key.note}
              className={`key ${key.isBlack ? 'black-key' : 'white-key'} ${!inScale ? 'disabled' : ''}`}
              style={dynamicStyle}
              onTouchStart={inScale ? handleStart : undefined}
              onTouchEnd={inScale ? handleEnd : undefined}
              onMouseDown={inScale ? handleStart : undefined}
              onMouseUp={inScale ? handleEnd : undefined}
              onMouseLeave={inScale ? handleEnd : undefined}
              disabled={!inScale}
            >
              <div className="key-note">{key.note}</div>
            </button>
          );
        })}
      </div>
    </div>
  );
};

export default Keyboard;

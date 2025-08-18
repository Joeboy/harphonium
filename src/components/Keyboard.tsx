import React, { useRef, useState } from 'react';
import './Keyboard.css';

interface KeyboardProps {
  onNoteStart: (frequency: number) => void;
  onNoteStop: () => void;
  onNoteDrag: (frequency: number) => Promise<void>;
  octaves: number;
  selectedKey: string;
  selectedScale: string;
  showNoteNames: boolean;
  transpose: number;
  displayDisabledNotes: boolean;
  keyboardType: 'keys' | 'fretless';
}

interface KeyData {
  frequency: number;
  note: string;
  isBlack?: boolean;
}

const Keyboard: React.FC<KeyboardProps> = ({
  onNoteStart,
  onNoteStop,
  onNoteDrag,
  octaves,
  selectedKey,
  selectedScale,
  showNoteNames,
  transpose,
  displayDisabledNotes,
  keyboardType,
}) => {
  const containerRef = useRef<HTMLDivElement>(null);

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
      ? Math.floor(numOctaves) * 12 + 1 // Complete octaves: C to C (13, 25, 37 keys)
      : Math.ceil(numOctaves * 12); // Fractional octaves: as before

    // Generate keys from high to low (C6 down to C4) for consistent ordering
    for (let i = totalKeys - 1; i >= 0; i--) {
      const octaveOffset = Math.floor(i / 12);
      const noteIndex = i % 12;
      const currentOctave = startOctave + octaveOffset;

      const baseNote = baseOctave[noteIndex];
      // Calculate frequency using the octave multiplier and transpose offset
      // Each octave doubles the frequency, and each semitone is 2^(1/12) ratio
      const frequency =
        baseNote.frequency * Math.pow(2, octaveOffset + transpose / 12);

      keys.push({
        frequency: frequency,
        note: `${baseNote.note}${currentOctave}`,
        isBlack: baseNote.isBlack,
      });
    }

    return keys;
  };

  // Restore scale filtering for display
  // Check if a note is in the selected scale
  const isNoteInScale = (note: string, key: string, scale: string): boolean => {
    if (scale === 'chromatic') return true;
    const noteWithoutOctave = note.replace(/\d+$/, '');
    const scaleIntervals: { [key: string]: number[] } = {
      major: [0, 2, 4, 5, 7, 9, 11],
      minor: [0, 2, 3, 5, 7, 8, 10],
      major_pentatonic: [0, 2, 4, 7, 9],
      minor_pentatonic: [0, 3, 5, 7, 10],
    };
    const noteToSemitone: { [key: string]: number } = {
      C: 0,
      'C#': 1,
      D: 2,
      'D#': 3,
      E: 4,
      F: 5,
      'F#': 6,
      G: 7,
      'G#': 8,
      A: 9,
      'A#': 10,
      B: 11,
    };
    const intervals = scaleIntervals[scale];
    if (!intervals) return true;
    const rootSemitone = noteToSemitone[key];
    const noteSemitone = noteToSemitone[noteWithoutOctave];
    if (rootSemitone === undefined || noteSemitone === undefined) return true;
    const interval = (noteSemitone - rootSemitone + 12) % 12;
    return intervals.includes(interval);
  };

  const keys = generateKeys(octaves);
  let filteredKeys = keys.map((key) => ({
    ...key,
    inScale: isNoteInScale(key.note, selectedKey, selectedScale),
  }));
  if (!displayDisabledNotes) {
    filteredKeys = filteredKeys.filter((key) => key.inScale);
  }

  // Calculate note pitch from horizontal position

  // Track the currently playing note index
  const [activeNoteIndex, setActiveNoteIndex] = useState<number | null>(null);
  const [isPointerDown, setIsPointerDown] = useState(false);

  // Helper to get note index and pitch from event
  const getNoteIndexAndPitchFromEvent = (
    e: React.TouchEvent | React.MouseEvent,
    playableKeys: typeof keys,
    keyboardType: 'keys' | 'fretless'
  ) => {
    const container = containerRef.current;
    if (!container) return { noteIndex: null, frequency: null };
    let clientY: number;
    if ('touches' in e && e.touches.length > 0) {
      clientY = e.touches[0].clientY;
    } else if ('clientY' in e) {
      clientY = e.clientY;
    } else {
      clientY = 0;
    }
    const rect = container.getBoundingClientRect();
    const y = clientY - rect.top;
    const height = rect.height;
    const pos = y / height;
    let noteIndex = Math.floor(pos * playableKeys.length);
    noteIndex = Math.max(0, Math.min(playableKeys.length - 1, noteIndex));

    if (keyboardType === 'fretless') {
      // Center of first key is at (0.5 / n), last key at (n-0.5)/n
      // Allow pitch to go slightly below/above the lowest/highest note at the very bottom/top
      const n = playableKeys.length;
      // Map pos in [0,1] to [-0.5, n-0.5] (so 0 is center of first key, 1 is center of last key)
      const minIdx = -0.5;
      const maxIdx = n - 0.5;
      const semitoneIndex = minIdx + (maxIdx - minIdx) * pos;
      // Clamp for noteIndex
      const lowerIndex = Math.floor(
        Math.max(0, Math.min(n - 2, semitoneIndex))
      );
      const upperIndex = lowerIndex + 1;
      const t = semitoneIndex - lowerIndex;
      // Get frequencies for lower and upper keys
      const lowerFreq = playableKeys[lowerIndex].frequency;
      const upperFreq = playableKeys[upperIndex]?.frequency ?? lowerFreq;
      // Interpolate in log2 space for perceptual accuracy
      const lowerLog = Math.log2(lowerFreq);
      const upperLog = Math.log2(upperFreq);
      const interpLog = lowerLog + (upperLog - lowerLog) * t;
      const frequency = Math.pow(2, interpLog);
      return { noteIndex: lowerIndex, frequency };
    } else {
      // Discrete keys
      return { noteIndex, frequency: playableKeys[noteIndex].frequency };
    }
  };

  const handleKeyboardStart = (e: React.TouchEvent | React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsPointerDown(true);
    const playableKeys = displayDisabledNotes ? keys : filteredKeys;
    const { noteIndex, frequency } = getNoteIndexAndPitchFromEvent(
      e,
      playableKeys,
      keyboardType
    );
    if (noteIndex !== null && frequency !== null) {
      const key = playableKeys[noteIndex];
      const inScale = isNoteInScale(key.note, selectedKey, selectedScale);
      if (inScale) {
        setTimeout(() => onNoteStart(frequency), 0);
        setActiveNoteIndex(noteIndex);
      }
    }
  };

  const handleKeyboardMove = (e: React.TouchEvent | React.MouseEvent) => {
    if (!isPointerDown) return;
    e.preventDefault();
    e.stopPropagation();
    const playableKeys = displayDisabledNotes ? keys : filteredKeys;
    const { noteIndex, frequency } = getNoteIndexAndPitchFromEvent(
      e,
      playableKeys,
      keyboardType
    );
    if (noteIndex !== null) {
      const key = playableKeys[noteIndex];
      const inScale = isNoteInScale(key.note, selectedKey, selectedScale);
      if (inScale) {
        if (noteIndex !== activeNoteIndex || keyboardType === 'fretless') {
          setTimeout(() => onNoteDrag(frequency!), 0);
          setActiveNoteIndex(noteIndex);
        }
      }
    }
  };

  const handleKeyboardEnd = (e: React.TouchEvent | React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsPointerDown(false);
    setActiveNoteIndex(null);
    setTimeout(() => onNoteStop(), 0);
  };

  return (
    <div
      className="keyboard-container"
      ref={containerRef}
      onTouchStart={handleKeyboardStart}
      onTouchEnd={handleKeyboardEnd}
      onTouchMove={handleKeyboardMove}
      onMouseDown={handleKeyboardStart}
      onMouseUp={handleKeyboardEnd}
      onMouseLeave={handleKeyboardEnd}
      onMouseMove={handleKeyboardMove}
      style={{ width: '100%', height: '100%' }}
    >
      <div className="keyboard" style={{ width: '100%', height: '100%' }}>
        {filteredKeys.map((key, idx) => {
          const dynamicStyle = {
            flex: key.isBlack ? '0.8' : '1',
            minHeight: key.isBlack ? '12px' : '15px',
          };
          // Show visually pressed key in keyboard mode
          const isActive = keyboardType === 'keys' && idx === activeNoteIndex;
          return (
            <button
              key={key.note}
              className={`key ${key.isBlack ? 'black-key' : 'white-key'}${
                !key.inScale ? ' disabled' : ''
              }${isActive ? ' active' : ''}`}
              style={{ ...dynamicStyle, pointerEvents: 'none' }}
              tabIndex={-1}
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

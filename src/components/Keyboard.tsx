import React, { useEffect, useRef, useState } from 'react';
import './Keyboard.css';

interface KeyboardProps {
  onNoteStart: (frequency: number) => void;
  onNoteStop: () => void;
  octaves: number;
}

interface KeyData {
  frequency: number;
  note: string;
  isBlack?: boolean;
}

const Keyboard: React.FC<KeyboardProps> = ({ onNoteStart, onNoteStop, octaves }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [keyHeight, setKeyHeight] = useState<number>(45);
  const [blackKeyHeight, setBlackKeyHeight] = useState<number>(36);

  // Calculate key sizes based on container height and number of octaves
  useEffect(() => {
    const calculateKeySizes = () => {
      if (!containerRef.current) return;

      const containerHeight = containerRef.current.clientHeight;
      const totalKeys = Math.ceil(octaves * 12);
      
      // Account for container padding, gaps between keys, and some buffer
      const containerPadding = 20; // 10px top + 10px bottom
      const keyboardPadding = 10; // 5px left/right padding in keyboard
      const gapSize = 3; // gap between keys
      const totalGaps = (totalKeys - 1) * gapSize;
      
      const availableHeight = containerHeight - containerPadding - keyboardPadding - totalGaps;
      
      // Calculate key height (white keys are the base, black keys are ~80% of white key height)
      const calculatedKeyHeight = Math.max(Math.floor(availableHeight / totalKeys), 20); // minimum 20px
      const calculatedBlackKeyHeight = Math.max(Math.floor(calculatedKeyHeight * 0.8), 16); // minimum 16px
      
      setKeyHeight(calculatedKeyHeight);
      setBlackKeyHeight(calculatedBlackKeyHeight);
    };

    calculateKeySizes();
    
    // Recalculate on window resize
    const handleResize = () => calculateKeySizes();
    window.addEventListener('resize', handleResize);
    
    return () => window.removeEventListener('resize', handleResize);
  }, [octaves]);

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
          const dynamicStyle = {
            minHeight: key.isBlack ? `${blackKeyHeight}px` : `${keyHeight}px`,
            height: key.isBlack ? `${blackKeyHeight}px` : `${keyHeight}px`,
          };
          
          return (
            <button
              key={key.note}
              className={`key ${key.isBlack ? 'black-key' : 'white-key'}`}
              style={dynamicStyle}
              onTouchStart={handleStart}
              onTouchEnd={handleEnd}
              onMouseDown={handleStart}
              onMouseUp={handleEnd}
              onMouseLeave={handleEnd}
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

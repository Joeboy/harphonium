import React from 'react';
import './Keyboard.css';

interface KeyboardProps {
  onNoteStart: (frequency: number) => void;
  onNoteStop: () => void;
}

interface KeyData {
  frequency: number;
  note: string;
  isBlack?: boolean;
}

const Keyboard: React.FC<KeyboardProps> = ({ onNoteStart, onNoteStop }) => {
  // Piano keys with frequencies (C3 to C5 range)
  const keys: KeyData[] = [
    { frequency: 1046.5, note: 'C6' },
    { frequency: 987.77, note: 'B5' },
    { frequency: 932.33, note: 'A#5', isBlack: true },
    { frequency: 880.0, note: 'A5' },
    { frequency: 830.61, note: 'G#5', isBlack: true },
    { frequency: 783.99, note: 'G5' },
    { frequency: 739.99, note: 'F#5', isBlack: true },
    { frequency: 698.46, note: 'F5' },
    { frequency: 659.25, note: 'E5' },
    { frequency: 622.25, note: 'D#5', isBlack: true },
    { frequency: 587.33, note: 'D5' },
    { frequency: 554.37, note: 'C#5', isBlack: true },
    { frequency: 523.25, note: 'C5' },
    { frequency: 493.88, note: 'B4' },
    { frequency: 466.16, note: 'A#4', isBlack: true },
    { frequency: 440.0, note: 'A4' },
    { frequency: 415.3, note: 'G#4', isBlack: true },
    { frequency: 392.0, note: 'G4' },
    { frequency: 369.99, note: 'F#4', isBlack: true },
    { frequency: 349.23, note: 'F4' },
    { frequency: 329.63, note: 'E4' },
    { frequency: 311.13, note: 'D#4', isBlack: true },
    { frequency: 293.66, note: 'D4' },
    { frequency: 277.18, note: 'C#4', isBlack: true },
    { frequency: 261.63, note: 'C4' },
    { frequency: 246.94, note: 'B3' },
    { frequency: 233.08, note: 'A#3', isBlack: true },
    { frequency: 220.0, note: 'A3' },
    { frequency: 207.65, note: 'G#3', isBlack: true },
    { frequency: 196.0, note: 'G3' },
    { frequency: 185.0, note: 'F#3', isBlack: true },
    { frequency: 174.61, note: 'F3' },
    { frequency: 164.81, note: 'E3' },
    { frequency: 155.56, note: 'D#3', isBlack: true },
    { frequency: 146.83, note: 'D3' },
    { frequency: 138.59, note: 'C#3', isBlack: true },
    { frequency: 130.81, note: 'C3' },
  ];

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
    <div className="keyboard-container">
      <div className="keyboard">
        {keys.map((key) => {
          const { handleStart, handleEnd } = createTouchHandlers(key.frequency);
          return (
            <button
              key={key.note}
              className={`key ${key.isBlack ? 'black-key' : 'white-key'}`}
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

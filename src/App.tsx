import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

// Declare global FastAudio interface for TypeScript
declare global {
  interface Window {
    FastAudio?: {
      playNote: (frequency: number) => void;
      stopNote: () => void;
      ping: () => string;
    };
  }
}

function App() {
  const [synthState, setSynthState] = useState<string>('');
  const [frequency, setFrequency] = useState<number>(440);
  const [hasFastAudio, setHasFastAudio] = useState<boolean>(false);

  useEffect(() => {
    // Check if fast audio interface is available (Android)
    if (window.FastAudio) {
      try {
        const response = window.FastAudio.ping();
        if (response === 'pong') {
          setHasFastAudio(true);
          setSynthState('✅ Fast audio interface ready');
          console.log('Fast audio interface detected and working');
        }
      } catch (error) {
        console.log('Fast audio interface not working:', error);
      }
    } else {
      setSynthState('Using Tauri IPC audio');
      console.log('Using fallback Tauri IPC audio');
    }
  }, []);

  // Optimized touch handlers for ultra-low latency
  const createTouchHandlers = (freq: number) => {
    const handleStart = (e: React.TouchEvent | React.MouseEvent) => {
      e.preventDefault(); // Prevent default touch behaviors that cause delays
      e.stopPropagation(); // Stop event bubbling
      setFrequency(freq);

      // Use setTimeout with 0 delay to break out of React's batching
      setTimeout(() => {
        if (hasFastAudio && window.FastAudio) {
          window.FastAudio.playNote(freq);
          setSynthState(`🚀 Fast playing: ${freq} Hz`);
        } else {
          playNote();
        }
      }, 0);
    };

    const handleEnd = (e: React.TouchEvent | React.MouseEvent) => {
      e.preventDefault();
      e.stopPropagation();
      setTimeout(() => stopNote(), 0);
    };

    return { handleStart, handleEnd };
  };

  async function playNote() {
    try {
      if (hasFastAudio && window.FastAudio) {
        // FAST PATH: Direct WebView->Native call (Android)
        console.log(`Fast audio play: ${frequency} Hz`);
        window.FastAudio.playNote(frequency);
        setSynthState(`🚀 Fast playing: ${frequency} Hz`);
      } else {
        // FALLBACK: Tauri IPC (desktop/other platforms)
        console.log(`Tauri audio play: ${frequency} Hz`);
        await invoke('play_note', { frequency: frequency });
        setSynthState(`🔊 Playing: ${frequency} Hz`);
      }
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
  }

  async function stopNote() {
    try {
      if (hasFastAudio && window.FastAudio) {
        // FAST PATH: Direct WebView->Native call (Android)
        console.log('Fast audio stop');
        window.FastAudio.stopNote();
        setSynthState('🚀 Fast stopped');
      } else {
        // FALLBACK: Tauri IPC (desktop/other platforms)
        console.log('Tauri audio stop');
        await invoke('stop_note');
        setSynthState('🔇 Stopped');
      }
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
  }

  return (
    <div className="container">
      <h1>SynthMob - Mobile Synthesizer</h1>

      <div className="status">
        <p>{synthState}</p>
        <small>
          {hasFastAudio ? '⚡ Low-latency mode active' : '🔄 Standard mode'}
        </small>
      </div>

      <div className="frequency-buttons">
        <button
          className="freq-btn"
          onTouchStart={createTouchHandlers(220).handleStart}
          onTouchEnd={createTouchHandlers(220).handleEnd}
          onMouseDown={createTouchHandlers(220).handleStart}
          onMouseUp={createTouchHandlers(220).handleEnd}
          style={{ touchAction: 'none' }} // Disable touch gestures that cause delays
        >
          220 Hz
        </button>

        <button
          className="freq-btn"
          onTouchStart={createTouchHandlers(440).handleStart}
          onTouchEnd={createTouchHandlers(440).handleEnd}
          onMouseDown={createTouchHandlers(440).handleStart}
          onMouseUp={createTouchHandlers(440).handleEnd}
          style={{ touchAction: 'none' }}
        >
          440 Hz
        </button>

        <button
          className="freq-btn"
          onTouchStart={createTouchHandlers(880).handleStart}
          onTouchEnd={createTouchHandlers(880).handleEnd}
          onMouseDown={createTouchHandlers(880).handleStart}
          onMouseUp={createTouchHandlers(880).handleEnd}
          style={{ touchAction: 'none' }}
        >
          880 Hz
        </button>
      </div>

      <div className="row">
        <div>
          <input
            id="frequency-input"
            type="number"
            value={frequency}
            onChange={(e) =>
              setFrequency(parseInt(e.currentTarget.value) || 440)
            }
            placeholder="Enter frequency..."
          />
          <label htmlFor="frequency-input">Custom Frequency (Hz)</label>
        </div>

        <button
          type="button"
          className="custom-btn"
          onTouchStart={createTouchHandlers(frequency).handleStart}
          onTouchEnd={createTouchHandlers(frequency).handleEnd}
          onMouseDown={createTouchHandlers(frequency).handleStart}
          onMouseUp={createTouchHandlers(frequency).handleEnd}
          style={{ touchAction: 'none' }}
        >
          Play Custom
        </button>
      </div>
    </div>
  );
}

export default App;

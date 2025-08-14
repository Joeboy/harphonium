import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [synthState, setSynthState] = useState<string>('');
  const [isAndroid, setIsAndroid] = useState<boolean>(false);

  useEffect(() => {
    // Detect if we're running on Android
    const userAgent = navigator.userAgent.toLowerCase();
    const androidDetected = userAgent.includes('android');

    setIsAndroid(androidDetected);

    if (androidDetected) {
      setSynthState(
        '🚀 Android: Optimized Tauri WebView with callback mode audio'
      );
      console.log(
        'Android detected - using optimized WebView with callback audio engine'
      );
    } else {
      setSynthState('💻 Desktop: Tauri with callback mode audio');
      console.log('Desktop mode - using Tauri with callback audio engine');
    }
  }, []);

  // Optimized touch handlers with minimal latency
  const createTouchHandlers = (freq: number) => {
    const handleStart = (e: React.TouchEvent | React.MouseEvent) => {
      e.preventDefault(); // Prevent default touch behaviors
      e.stopPropagation(); // Stop event bubbling

      // Use setTimeout with 0 delay to break out of React's batching for faster execution
      setTimeout(() => playNote(freq), 0);
    };

    const handleEnd = (e: React.TouchEvent | React.MouseEvent) => {
      e.preventDefault();
      e.stopPropagation();
      setTimeout(() => stopNote(), 0);
    };

    return { handleStart, handleEnd };
  };

  async function playNote(frequency: number) {
    try {
      console.log(`Playing note: ${frequency} Hz`);
      await invoke('play_note', { frequency: frequency });
      setSynthState(`🔊 Playing: ${frequency} Hz`);
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
  }

  async function stopNote() {
    try {
      console.log('Stopping note');
      await invoke('stop_note');
      setSynthState('🔇 Stopped');
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
          {isAndroid
            ? '⚡ Callback mode audio engine'
            : '🔄 Desktop callback mode'}
        </small>
      </div>

      <div className="frequency-buttons">
        <button
          className="freq-btn"
          onTouchStart={createTouchHandlers(220).handleStart}
          onTouchEnd={createTouchHandlers(220).handleEnd}
          onMouseDown={createTouchHandlers(220).handleStart}
          onMouseUp={createTouchHandlers(220).handleEnd}
          style={{ touchAction: 'none' }}
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
    </div>
  );
}

export default App;

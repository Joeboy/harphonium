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
          onMouseDown={() => {
            setFrequency(220);
            playNote();
          }}
          onMouseUp={stopNote}
          onTouchStart={() => {
            setFrequency(220);
            playNote();
          }}
          onTouchEnd={stopNote}
        >
          220 Hz
        </button>

        <button
          className="freq-btn"
          onMouseDown={() => {
            setFrequency(440);
            playNote();
          }}
          onMouseUp={stopNote}
          onTouchStart={() => {
            setFrequency(440);
            playNote();
          }}
          onTouchEnd={stopNote}
        >
          440 Hz
        </button>

        <button
          className="freq-btn"
          onMouseDown={() => {
            setFrequency(880);
            playNote();
          }}
          onMouseUp={stopNote}
          onTouchStart={() => {
            setFrequency(880);
            playNote();
          }}
          onTouchEnd={stopNote}
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
          onMouseDown={playNote}
          onMouseUp={stopNote}
          onTouchStart={playNote}
          onTouchEnd={stopNote}
        >
          Play Custom
        </button>
      </div>
    </div>
  );
}

export default App;

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Keyboard from './components/Keyboard';
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
    <div className="app-container">
      <div className="main-content">
        <div className="info-panel">
          <h1>SynthMob</h1>
          <p className="subtitle">Mobile Synthesizer</p>

          <div className="status">
            <p>{synthState}</p>
            <small>
              {isAndroid
                ? '⚡ Callback mode audio engine'
                : '🔄 Desktop callback mode'}
            </small>
          </div>

          <div className="instructions">
            <h3>How to Play</h3>
            <ul>
              <li>Touch and hold keys to play notes</li>
              <li>Release to stop the sound</li>
              <li>Scroll the keyboard for more octaves</li>
              <li>Each key shows note name and frequency</li>
            </ul>
          </div>
        </div>
      </div>

      <div className="keyboard-panel">
        <Keyboard onNoteStart={playNote} onNoteStop={stopNote} />
      </div>
    </div>
  );
}

export default App;

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Keyboard from './components/Keyboard';
import KeyboardTab from './components/KeyboardTab';
import SynthTab from './components/SynthTab';
import EffectsTab from './components/EffectsTab';
import './App.css';

type TabType = 'info' | 'keyboard' | 'synth' | 'effects';

function App() {
  const [synthState, setSynthState] = useState<string>('');
  const [isAndroid, setIsAndroid] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState<TabType>('info');
  const [octaves, setOctaves] = useState<number>(1.5);
  const [scaleSettings, setScaleSettings] = useState({
    selectedKey: 'C',
    selectedScale: 'chromatic',
    showNoteNames: true,
    transpose: 0,
    displayDisabledNotes: true,
  });

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

  const renderTabContent = () => {
    switch (activeTab) {
      case 'info':
        return (
          <div className="tab-content">
            <h1>Harphonium</h1>
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
        );

      case 'keyboard':
        return (
          <KeyboardTab
            octaves={octaves}
            onOctavesChange={setOctaves}
            scaleSettings={scaleSettings}
            onScaleSettingsChange={setScaleSettings}
          />
        );

      case 'synth':
        return <SynthTab />;

      case 'effects':
        return <EffectsTab isActive={activeTab === 'effects'} />;
    }
  };

  return (
    <div className="app-container">
      <div className="left-pane">
        <div className="tab-bar">
          <button
            className={`tab ${activeTab === 'info' ? 'active' : ''}`}
            onClick={() => setActiveTab('info')}
          >
            Info
          </button>
          <button
            className={`tab ${activeTab === 'keyboard' ? 'active' : ''}`}
            onClick={() => setActiveTab('keyboard')}
          >
            Keyboard
          </button>
          <button
            className={`tab ${activeTab === 'synth' ? 'active' : ''}`}
            onClick={() => setActiveTab('synth')}
          >
            Synth
          </button>
          <button
            className={`tab ${activeTab === 'effects' ? 'active' : ''}`}
            onClick={() => setActiveTab('effects')}
          >
            Effects
          </button>
        </div>
        <div className="tab-content-container">{renderTabContent()}</div>
      </div>

      <div className="right-pane">
        <Keyboard
          onNoteStart={playNote}
          onNoteStop={stopNote}
          octaves={octaves}
          selectedKey={scaleSettings.selectedKey}
          selectedScale={scaleSettings.selectedScale}
          showNoteNames={scaleSettings.showNoteNames}
          transpose={scaleSettings.transpose}
          displayDisabledNotes={scaleSettings.displayDisabledNotes}
        />
      </div>
    </div>
  );
}

export default App;

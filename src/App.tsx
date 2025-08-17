import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Keyboard from './components/Keyboard';
import KeyboardTab from './components/KeyboardTab';
import SynthTab from './components/SynthTab';
import EffectsTab from './components/EffectsTab';
import './App.css';

type TabType = 'synth' | 'keyboard' | 'effects' | 'info';

function App() {
  const [synthState, setSynthState] = useState<string>('');
  const [isAndroid, setIsAndroid] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState<TabType>('keyboard');
  const [octaves, setOctaves] = useState<number>(1.5);
  const [scaleSettings, setScaleSettings] = useState({
    selectedKey: 'C',
    selectedScale: 'chromatic',
    showNoteNames: true,
    transpose: 0,
    displayDisabledNotes: true,
  });
  // keyboardType state is now lifted to App
  const [keyboardType, setKeyboardType] = useState<'keys' | 'fretless'>('keys');

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
      const freqStr = frequency.toFixed(2).replace(/\.00$/, '');
      console.log(`Playing note: ${freqStr} Hz`);
      await invoke('play_note', { frequency: frequency });
      setSynthState(`🔊 Playing: ${freqStr} Hz`);
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
  }

  async function setFrequency(frequency: number) {
    await invoke('set_frequency', { frequency });
  }

  async function noteOff() {
    try {
      console.log('Stopping note');
      await invoke('note_off');
      setSynthState('🔇 No keys down');
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
  }

  // Scroll tab-content-container to top on tab change
  useEffect(() => {
    const container = document.querySelector('.tab-content-container');
    if (container) {
      container.scrollTop = 0;
    }
  }, [activeTab]);

  const renderTabContent = () => {
    switch (activeTab) {
      case 'keyboard':
        return (
          <KeyboardTab
            octaves={octaves}
            onOctavesChange={setOctaves}
            scaleSettings={scaleSettings}
            onScaleSettingsChange={setScaleSettings}
            keyboardType={keyboardType}
            onKeyboardTypeChange={setKeyboardType}
          />
        );

      case 'synth':
        return <SynthTab />;

      case 'effects':
        return <EffectsTab isActive={activeTab === 'effects'} />;

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
          </div>
        );
    }
  };

  return (
    <div className="app-container">
      <div className="left-pane">
        <div className="tab-bar">
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

          <button
            className={`tab ${activeTab === 'info' ? 'active' : ''}`}
            onClick={() => setActiveTab('info')}
          >
            Info
          </button>
        </div>
        <div className="tab-content-container">{renderTabContent()}</div>
      </div>

      <div className="right-pane">
        <Keyboard
          onNoteStart={playNote}
          onNoteStop={noteOff}
          onNoteDrag={setFrequency}
          octaves={octaves}
          selectedKey={scaleSettings.selectedKey}
          selectedScale={scaleSettings.selectedScale}
          showNoteNames={scaleSettings.showNoteNames}
          transpose={scaleSettings.transpose}
          displayDisabledNotes={scaleSettings.displayDisabledNotes}
          keyboardType={keyboardType}
        />
      </div>
    </div>
  );
}

export default App;

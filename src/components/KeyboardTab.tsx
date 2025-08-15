import React, { useState } from 'react';
import './KeyboardTab.css';

interface KeyboardTabProps {
  octaves: number;
  onOctavesChange: (octaves: number) => void;
  onScaleSettingsChange: (settings: {
    selectedKey: string;
    selectedScale: string;
    showNoteNames: boolean;
    transpose: number;
    displayDisabledNotes: boolean;
  }) => void;
}

const KeyboardTab: React.FC<KeyboardTabProps> = ({
  octaves,
  onOctavesChange,
  onScaleSettingsChange,
}) => {
  const [selectedKey, setSelectedKey] = useState('C');
  const [selectedScale, setSelectedScale] = useState('chromatic');
  const [showNoteNames, setShowNoteNames] = useState(true);
  const [transpose, setTranspose] = useState(0);
  const [displayDisabledNotes, setDisplayDisabledNotes] = useState(true);
  const [keyboardType, setKeyboardType] = useState<'keys' | 'slide' | 'fretless'>('keys');
  const handleKeyboardTypeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setKeyboardType(e.target.value as 'keys' | 'slide' | 'fretless');
  };

  const handleOctavesChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onOctavesChange(parseFloat(e.target.value));
  };

  const handleKeyChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newKey = e.target.value;
    setSelectedKey(newKey);
    onScaleSettingsChange({
      selectedKey: newKey,
      selectedScale,
      showNoteNames,
      transpose,
      displayDisabledNotes,
    });
  };

  const handleScaleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newScale = e.target.value;
    setSelectedScale(newScale);
    onScaleSettingsChange({
      selectedKey,
      selectedScale: newScale,
      showNoteNames,
      transpose,
      displayDisabledNotes,
    });
  };

  const handleShowNoteNamesChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newShowNoteNames = e.target.checked;
    setShowNoteNames(newShowNoteNames);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames: newShowNoteNames,
      transpose,
      displayDisabledNotes,
    });
  };

  const handleDisplayDisabledNotesChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newDisplayDisabledNotes = e.target.checked;
    setDisplayDisabledNotes(newDisplayDisabledNotes);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose,
      displayDisabledNotes: newDisplayDisabledNotes,
    });
  };

  const handleTransposeUp = () => {
    const newTranspose = transpose === 24 ? -24 : transpose + 1;
    setTranspose(newTranspose);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose: newTranspose,
      displayDisabledNotes,
    });
  };

  const handleTransposeDown = () => {
    const newTranspose = transpose === -24 ? 24 : transpose - 1;
    setTranspose(newTranspose);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose: newTranspose,
      displayDisabledNotes,
    });
  };

  const handleOctaveUp = () => {
    // Add 12 semitones (1 octave) - max +24
    const newTranspose = Math.min(transpose + 12, 24);
    setTranspose(newTranspose);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose: newTranspose,
      displayDisabledNotes,
    });
  };

  const handleOctaveDown = () => {
    // Subtract 12 semitones (1 octave) - min -24
    const newTranspose = Math.max(transpose - 12, -24);
    setTranspose(newTranspose);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose: newTranspose,
      displayDisabledNotes,
    });
  };

  return (
    <div className="tab-content">
      <div className="settings-section">
        <div className="setting-item">
          <label>Keyboard type</label>
          <div>
            <label style={{ marginRight: '1em' }}>
              <input
                type="radio"
                name="keyboard-type"
                value="keys"
                checked={keyboardType === 'keys'}
                onChange={handleKeyboardTypeChange}
              />
              Keys
            </label>
            <label style={{ marginRight: '1em' }}>
              <input
                type="radio"
                name="keyboard-type"
                value="slide"
                checked={keyboardType === 'slide'}
                onChange={handleKeyboardTypeChange}
              />
              Slide
            </label>
            <label>
              <input
                type="radio"
                name="keyboard-type"
                value="fretless"
                checked={keyboardType === 'fretless'}
                onChange={handleKeyboardTypeChange}
              />
              Fretless
            </label>
          </div>
        </div>
        <div className="setting-item">
          <label>Octaves</label>
          <select value={octaves} onChange={handleOctavesChange}>
            <option value="1">1 Octave</option>
            <option value="1.5">1.5 Octaves</option>
            <option value="2">2 Octaves</option>
            <option value="2.5">2.5 Octaves</option>
            <option value="3">3 Octaves</option>
          </select>
        </div>
        <div className="setting-item">
          <label>Key</label>
          <select value={selectedKey} onChange={handleKeyChange}>
            <option value="C">C</option>
            <option value="C#">C#</option>
            <option value="D">D</option>
            <option value="D#">D#</option>
            <option value="E">E</option>
            <option value="F">F</option>
            <option value="F#">F#</option>
            <option value="G">G</option>
            <option value="G#">G#</option>
            <option value="A">A</option>
            <option value="A#">A#</option>
            <option value="B">B</option>
          </select>
        </div>
        <div className="setting-item">
          <label>Scale</label>
          <select value={selectedScale} onChange={handleScaleChange}>
            <option value="chromatic">Chromatic</option>
            <option value="major">Major</option>
            <option value="minor">Minor</option>
            <option value="major_pentatonic">Major Pentatonic</option>
            <option value="minor_pentatonic">Minor Pentatonic</option>
          </select>
        </div>
        <div className="setting-item">
          <label>
            <input 
              type="checkbox" 
              checked={showNoteNames}
              onChange={handleShowNoteNamesChange}
            />
            Show Note Names
          </label>
        </div>
        <div className="setting-item">
          <label>
            <input 
              type="checkbox" 
              checked={displayDisabledNotes}
              onChange={handleDisplayDisabledNotesChange}
            />
            Display Disabled Notes
          </label>
        </div>
        <div className="setting-item">
          <label>Transpose: {transpose > 0 ? `+${transpose}` : transpose} semitones</label>
          <div className="transpose-controls">
            <button 
              type="button"
              className="transpose-button"
              onClick={handleTransposeDown}
            >
              ▼ Down
            </button>
            <button 
              type="button"
              className="transpose-button"
              onClick={handleTransposeUp}
            >
              ▲ Up
            </button>
          </div>
          <div className="transpose-controls" style={{ marginTop: '8px' }}>
            <button 
              type="button"
              className="transpose-button"
              onClick={handleOctaveDown}
            >
              ▼▼ Octave Down
            </button>
            <button 
              type="button"
              className="transpose-button"
              onClick={handleOctaveUp}
            >
              ▲▲ Octave Up
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default KeyboardTab;

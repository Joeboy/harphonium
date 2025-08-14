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
  }) => void;
}

const KeyboardTab: React.FC<KeyboardTabProps> = ({
  octaves,
  onOctavesChange,
  onScaleSettingsChange,
}) => {
  const [selectedKey, setSelectedKey] = useState<string>('C');
  const [selectedScale, setSelectedScale] = useState<string>('chromatic');
  const [showNoteNames, setShowNoteNames] = useState<boolean>(true);
  const [transpose, setTranspose] = useState<number>(0);

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
    });
  };

  const handleTransposeUp = () => {
    const newTranspose = transpose === 24 ? -24 : transpose + 1;
    setTranspose(newTranspose);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose: newTranspose
    });
  };

  const handleTransposeDown = () => {
    const newTranspose = transpose === -24 ? 24 : transpose - 1;
    setTranspose(newTranspose);
    onScaleSettingsChange({
      selectedKey,
      selectedScale,
      showNoteNames,
      transpose: newTranspose
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
      transpose: newTranspose
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
      transpose: newTranspose
    });
  };

  return (
    <div className="tab-content">
      <div className="settings-section">
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

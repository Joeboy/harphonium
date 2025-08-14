import React from 'react';
import './KeyboardTab.css';

interface KeyboardTabProps {
  octaves: number;
  onOctavesChange: (octaves: number) => void;
  selectedKey: string;
  onKeyChange: (key: string) => void;
  selectedScale: string;
  onScaleChange: (scale: string) => void;
}

const KeyboardTab: React.FC<KeyboardTabProps> = ({
  octaves,
  onOctavesChange,
  selectedKey,
  onKeyChange,
  selectedScale,
  onScaleChange,
}) => {
  const handleOctavesChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onOctavesChange(parseFloat(e.target.value));
  };

  const handleKeyChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onKeyChange(e.target.value);
  };

  const handleScaleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onScaleChange(e.target.value);
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
            <input type="checkbox" defaultChecked />
            Show Note Names
          </label>
        </div>
      </div>
    </div>
  );
};

export default KeyboardTab;

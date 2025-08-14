import React from 'react';
import './KeyboardTab.css';

interface KeyboardTabProps {
  octaves: number;
  onOctavesChange: (octaves: number) => void;
}

const KeyboardTab: React.FC<KeyboardTabProps> = ({ octaves, onOctavesChange }) => {
  const handleOctavesChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onOctavesChange(parseFloat(e.target.value));
  };

  return (
    <div className="tab-content">
      <h2>Keyboard</h2>
      <div className="settings-section">
        <h3>Audio Settings</h3>
        <div className="setting-item">
          <label>Master Volume</label>
          <input type="range" min="0" max="100" defaultValue="40" />
        </div>
        <div className="setting-item">
          <label>Reverb Amount</label>
          <input type="range" min="0" max="100" defaultValue="30" />
        </div>
      </div>
      <div className="settings-section">
        <h3>Keyboard Settings</h3>
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
          <label>
            <input type="checkbox" defaultChecked />
            Show Note Names
          </label>
        </div>
        <div className="setting-item">
          <label>
            <input type="checkbox" defaultChecked />
            Show Frequencies
          </label>
        </div>
      </div>
    </div>
  );
};

export default KeyboardTab;

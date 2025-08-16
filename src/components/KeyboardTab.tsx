import React from 'react';

interface KeyboardTabProps {
  octaves: number;
  onOctavesChange: (octaves: number) => void;
  scaleSettings: {
    selectedKey: string;
    selectedScale: string;
    showNoteNames: boolean;
    transpose: number;
    displayDisabledNotes: boolean;
  };
  onScaleSettingsChange: (settings: {
    selectedKey: string;
    selectedScale: string;
    showNoteNames: boolean;
    transpose: number;
    displayDisabledNotes: boolean;
  }) => void;
  keyboardType: 'keys' | 'fretless';
  onKeyboardTypeChange: (type: 'keys' | 'fretless') => void;
}

const KeyboardTab: React.FC<KeyboardTabProps> = ({
  octaves,
  onOctavesChange,
  scaleSettings,
  onScaleSettingsChange,
  keyboardType,
  onKeyboardTypeChange,
}) => {
  const handleKeyboardTypeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onKeyboardTypeChange(e.target.value as 'keys' | 'fretless');
  };

  const handleOctavesChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onOctavesChange(parseFloat(e.target.value));
  };

  const handleKeyChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onScaleSettingsChange({
      ...scaleSettings,
      selectedKey: e.target.value,
    });
  };

  const handleScaleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onScaleSettingsChange({
      ...scaleSettings,
      selectedScale: e.target.value,
    });
  };

  const handleShowNoteNamesChange = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    onScaleSettingsChange({
      ...scaleSettings,
      showNoteNames: e.target.checked,
    });
  };

  const handleDisplayDisabledNotesChange = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    onScaleSettingsChange({
      ...scaleSettings,
      displayDisabledNotes: e.target.checked,
    });
  };

  const handleTransposeUp = () => {
    const newTranspose =
      scaleSettings.transpose === 24 ? -24 : scaleSettings.transpose + 1;
    onScaleSettingsChange({
      ...scaleSettings,
      transpose: newTranspose,
    });
  };

  const handleTransposeDown = () => {
    const newTranspose =
      scaleSettings.transpose === -24 ? 24 : scaleSettings.transpose - 1;
    onScaleSettingsChange({
      ...scaleSettings,
      transpose: newTranspose,
    });
  };

  const handleOctaveUp = () => {
    // Add 12 semitones (1 octave) - max +24
    const newTranspose = Math.min(scaleSettings.transpose + 12, 24);
    onScaleSettingsChange({
      ...scaleSettings,
      transpose: newTranspose,
    });
  };

  const handleOctaveDown = () => {
    // Subtract 12 semitones (1 octave) - min -24
    const newTranspose = Math.max(scaleSettings.transpose - 12, -24);
    onScaleSettingsChange({
      ...scaleSettings,
      transpose: newTranspose,
    });
  };

  return (
    <div className="tab-content">
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
        <label>Octaves</label>
        <select value={octaves} onChange={handleOctavesChange}>
          <option value="1">1 Octave</option>
          <option value="1.5">1.5 Octaves</option>
          <option value="2">2 Octaves</option>
          <option value="2.5">2.5 Octaves</option>
          <option value="3">3 Octaves</option>
        </select>
        <label>Key</label>
        <select value={scaleSettings.selectedKey} onChange={handleKeyChange}>
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
        <label>Scale</label>
        <select
          value={scaleSettings.selectedScale}
          onChange={handleScaleChange}
        >
          <option value="chromatic">Chromatic</option>
          <option value="major">Major</option>
          <option value="minor">Minor</option>
          <option value="major_pentatonic">Major Pentatonic</option>
          <option value="minor_pentatonic">Minor Pentatonic</option>
        </select>
        <label>
          <input
            type="checkbox"
            checked={scaleSettings.showNoteNames}
            onChange={handleShowNoteNamesChange}
          />
          Show Note Names
        </label>
        <label>
          <input
            type="checkbox"
            checked={scaleSettings.displayDisabledNotes}
            onChange={handleDisplayDisabledNotesChange}
          />
          Display Disabled Notes
        </label>
        <label>
          Transpose:{' '}
          {scaleSettings.transpose > 0
            ? `+${scaleSettings.transpose}`
            : scaleSettings.transpose}{' '}
          semitones
        </label>
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
  );
};

export default KeyboardTab;

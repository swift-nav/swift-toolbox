.pragma library

// TODO(JV) CPP-116: Generate from Capnp definition, or define in common constants
const obsColKeys = [
  'prn',
  'pseudoRange',
  'carrierPhase',
  'cn0',
  'measuredDoppler',
  'computedDoppler',
  'lock',
  'flags',
];

const obsColNames = [
  'PRN',
  'Pseudorange (m)',
  'Carrier Phase (cycles)',
  'C/N0 (dB-Hz)',
  'Meas. Doppler (Hz)',
  'Comp. Doppler (Hz)',
  'Lock',
  'Flags',
];

const obsSampleData = [{
  'prn': '1 (GPS L1CA)',
  'pseudoRange': 20855933.3634534,
  'carrierPhase': 11112345.3634534,
  'cn0': 50.8,
  'measuredDoppler': 1823.3634534,
  'computedDoppler': 1821.3634534,
  'lock': '15',
  'flags': '0x000F = PR CP 1/2C MD',
}, {
  'prn': '1 (GPS L2C M)',
  'pseudoRange': 20855934.08,
  'carrierPhase': 86652045.3634534,
  'cn0': 49.5,
  'measuredDoppler': 1410.3634534,
  'computedDoppler': 1411.3634534,
  'lock': '15',
  'flags': '0x000F = PR CP 1/2C MD',
}, {
  'prn': '1 (GPS L1CA)',
  'pseudoRange': 23231967.92,
  'carrierPhase': 124900123.3634534,
  'cn0': 41.8,
  'measuredDoppler': 2822.3634534,
  'computedDoppler': 2821.3634534,
  'lock': '15',
  'flags': '0x000F = PR CP 1/2C MD',
}];

function getObsCell(model, modelIndex) {
  const key = obsColKeys[modelIndex.column];
  return model.rows[modelIndex.row][key];
}

function padFloat(num, length, digits=2, allowNegative = true) {
  if (!num) {
    return '--';
  }
  const s = num.toFixed(digits);
  if (!allowNegative) {
    return s.padStart(length);
  } else {
    return s.padStart(length + 1);
  }
}

function showFlags(flags) {
  if (!flags) {
    return '0x0000';
  }
  let flagStr = '0x' + flags.toString(16).padStart(4, '0') + ' =';

  // Bit 0 is Pseudorange valid
  if (flags & 0x01) {
    flagStr += ' PR';
  }
  // Bit 1 is Carrier phase valid
  if (flags & 0x02) {
    flagStr += ' CP';
  }
  // Bit 2 is Half-cycle ambiguity
  if (flags & 0x04) {
    flagStr += ' 1/2C';
  }
  // Bit 3 is Measured Doppler Valid
  if (flags & 0x08) {
    flagStr += ' MD';
  }
  return flagStr;
}

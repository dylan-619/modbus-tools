export type AddressMode = 'Protocol' | 'PLC';

/**
 * Parses the user input address based on the addressing mode and function code.
 * 
 * @param input The user input, can be a number or string like '40063'
 * @param mode The addressing mode: 'Protocol' (Base 0) or 'PLC' (Base 1 / Absolute)
 * @param functionCode The Modbus function code (1, 2, 3, 4, 5, 6, 15, 16)
 * @returns The 0-based protocol wire address
 */
export function parseModbusAddress(input: number | string, mode: AddressMode, functionCode: number): number {
  let val = typeof input === 'string' ? parseInt(input, 10) : input;
  if (isNaN(val)) return 0;

  if (mode === 'Protocol') {
    return val;
  }

  // PLC Mode (Base 1 or Absolute)
  // If it's a 5 or 6 digit number, we try to strip the region prefix.
  if (val >= 10000) {
    const valStr = val.toString();
    const prefix = parseInt(valStr.charAt(0), 10);
    const offset = parseInt(valStr.substring(1), 10);

    // Validate prefix against function code
    let validPrefix = false;
    switch (functionCode) {
      case 1:
      case 5:
      case 15:
        if (prefix === 0) {
          // Coils usually start with 0xxxx, but 00063 parses as 63, so it falls into the offset < 10000 branch below.
          validPrefix = true; 
        }
        break;
      case 2:
        if (prefix === 1) validPrefix = true; // 1xxxx Inputs
        break;
      case 4:
        if (prefix === 3) validPrefix = true; // 3xxxx Input Registers
        break;
      case 3:
      case 6:
      case 16:
        if (prefix === 4) validPrefix = true; // 4xxxx Holding Registers
        break;
    }

    if (validPrefix && offset > 0) {
      return offset - 1; // Base 1 to Base 0
    }
  }

  // If it's just an offset (e.g., 63), subtract 1
  return Math.max(0, val - 1);
}

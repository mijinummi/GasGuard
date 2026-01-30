import { GasGuardEngine } from '../packages/rules/gasGuard/gasguard.engine'
import * as path from 'path';

describe('Soroban Full Scan Lifecycle', () => {
  const fixturesDir = path.join(
    __dirname,
    '../../../fixtures/soroban',
  );

  const expectedDir = path.join(fixturesDir, 'expected');

  let engine: GasGuardEngine;

  beforeAll(() => {
    engine = new GasGuardEngine();
  });

  it.each([
    'inefficient_storage.rs',
    'inefficient_loop.rs',
    'redundant_clone.rs',
  ])('scans %s and returns expected findings', async (file) => {
    const source = fs.readFileSync(
      path.join(fixturesDir, file),
      'utf8',
    );

    const expected = JSON.parse(
      fs.readFileSync(
        path.join(expectedDir, file.replace('.rs', '.json')),
        'utf8',
      ),
    );

    const result = await engine.scan({
      language: 'soroban',
      source,
    });

    expect(result.issues).toEqual(expected.issues);
  });
});

import { noOutput, hasOutput } from 'native';
import { test } from '@playwright/test';

test('no console output', () => {
  noOutput();
});

test('has console output', () => {
  hasOutput();
});

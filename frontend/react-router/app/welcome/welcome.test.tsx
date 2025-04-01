// Used for __tests__/testing-library.js
// Learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom/vitest';

import { render, screen } from '@testing-library/react';
import { Welcome } from "./welcome";
import { it, describe, expect } from 'vitest';

/**
* @vitest-environment jsdom
*/
describe('app', () => {
  it('renders hello world', () => {
    render(<Welcome />);
    const linkElement = screen.getByText(/What's next\?/i);
    expect(linkElement).toBeInTheDocument();
  })
});
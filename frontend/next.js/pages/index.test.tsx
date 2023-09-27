import { render, screen } from '@testing-library/react';
import Home from './index';

// Used for __tests__/testing-library.js
// Learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom/extend-expect';

describe('Home', () => {
  it('renders a heading', () => {
    render(<Home />);

    const heading = screen.getByRole('heading', {
      name: /welcome to next\.js!/i,
    });

    expect(heading).toBeInTheDocument();
  });
});

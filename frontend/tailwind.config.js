/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        canvas: '#f7f8f6',
        surface: '#fbfcfa',
        panel: '#ffffff',
        line: '#dfe4dc',
        ink: {
          DEFAULT: '#1f2933',
          muted: '#66736b',
          soft: '#879088',
        },
        accent: {
          DEFAULT: '#5f7f6b',
          soft: '#eef3ea',
          strong: '#365141',
        },
      },
      fontFamily: {
        sans: [
          'Inter',
          'ui-sans-serif',
          'system-ui',
          '-apple-system',
          'BlinkMacSystemFont',
          'Segoe UI',
          'sans-serif',
        ],
      },
      boxShadow: {
        subtle: '0 1px 2px rgb(31 41 51 / 0.06)',
      },
      borderRadius: {
        ui: '0.5rem',
      },
    },
  },
  plugins: [],
};

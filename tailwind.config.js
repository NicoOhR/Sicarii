/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/*.html'],
  theme: {
    extend: {
      fontFamily: {
        wittgenstein: ['"Wittgenstein"', 'serif'],
      },
      transform: ['hover'],
      scale: ['hover'],
    },
  },
  plugins: [],
}


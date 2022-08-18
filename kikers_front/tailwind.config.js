/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*"],
  theme: {
    extend: {
	gridRow: {
        	'span-9': 'span 9 / span 9',
      	}
    },
  },
  plugins: [],
}

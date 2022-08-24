/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*"],
  theme: {
    extend: {
	gridRow: {
        	'span-10': 'span 10 / span 10',
        	'span-11': 'span 11 / span 11',
        	'span-4': 'span 4 / span 4',
      	}
    },
  },
  plugins: [],
}

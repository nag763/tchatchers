/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*"],
  theme: {
	fontFamily : {
		'tahoma': ['Tahoma', 'ui-serif', 'sans-serif']
	},
    extend: {
		gridTemplateRows: {
			'12': 'repeat(12, minmax(0, 1fr))',
		},
		gridRow: {
				'span-10': 'span 10 / span 10',
				'span-11': 'span 11 / span 11',
				'span-4': 'span 4 / span 4',
			}
		},
  },
  plugins: [],
}

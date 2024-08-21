FizzBuzz: [
  fizzBuzz! num* [
    result3% >checkIfWhole num#>div '3'#*<*<
    result5% >checkIfWhole num#>div '5'#*<*<

    
    result3>and result5*<>then [
      'FizzBuzz'$
    ]*<>lest result3>then [
      'Fizz'$
    ]*<*<>lest result5>then [
      'Buzz'$
    ]*<*<>lest [
      num#
    ]*<
  ]

  checkIfWhole! num* [
    num>equals num>round<*<<
  ]
]

fizzBuzz% FizzBuzz

'20'#|>for 'i'$* [fizzBuzz>fizzBuzz i*<]* <

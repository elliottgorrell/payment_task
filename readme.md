# Interview Task

Overall everything should be fairly straight forward. Some design decision I made:

* Rather than unit testing I just added some high level integration testing. Due to a few factors:
  * Time limit on task
  * Most transacation types are lacking complexity that is strictly needed and rust type system will be good enough for now. I consider this task a POC project and this gives far more flexibility for refactoring and changing the design over the initial period of work
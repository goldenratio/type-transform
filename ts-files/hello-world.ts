/**
 * Hello World!!
 */
interface HelloWorld {
  /**
   * ID
   */
  readonly id: string;

  readonly user: User;

  readonly enabled: boolean;
  /**
   * Say's hello
   */
  sayHello(): void;
}

interface User {
  readonly name: string;
}

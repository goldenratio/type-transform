export declare interface TransformOptions {
  /**
   * A banner to be added to the generated file, this can be a package path for "kotlin",
   * a custom auto code generated message or a comment block such as a license for the code
   */
  readonly banner?: string;

  /**
   * A footer to be added to the generated file, this can be something like a comment block for a license
   * or just a fun easter egg
   */
  readonly footer?: string;
}

export declare interface TransformResult {
  readonly success: boolean;
}

/**
 * Transform TypeScript types to Swift/Kotlin types
 */
export declare function transform(srcFilePath: string, outFilePath: string, options: TransformOptions): Promise<TransformResult>;

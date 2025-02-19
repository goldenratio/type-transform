export declare interface TrasnformOptions {
  readonly banner?: string;
  readonly footer?: string;
}

export declare interface TransformResult {
  readonly success: boolean;
}

export declare function transform(srcFilePath: string, outFilePath: string, options: TransformOptions): Promise<TransformResult>;

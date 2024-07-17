// generated by diplomat-tool
import type { ImportedStruct } from "./ImportedStruct"
import type { MyStruct } from "./MyStruct"
import type { pointer, char } from "./diplomat-runtime.d.ts";

export class Opaque {
    

    get ffiValue(): pointer;


    static new_(): Opaque;

    static tryFromUtf8(input: string): Opaque | undefined;

    static fromStr(input: string): Opaque;

    getDebugStr(): string;

    assertStruct(s: MyStruct): void;

    static returnsUsize(): number;

    static returnsImported(): ImportedStruct;

    static cmp(): number;

    

}
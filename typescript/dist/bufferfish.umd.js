!function(t,i){"object"==typeof exports&&"undefined"!=typeof module?i(exports):"function"==typeof define&&define.amd?define(["exports"],i):i((t="undefined"!=typeof globalThis?globalThis:t||self).bufferfish={})}(this,(function(t){"use strict";t.Bufferfish=class{constructor(t=new ArrayBuffer(0)){this.view=()=>this.inner.slice(),this.writeUint8=t=>{if(t>255||t<0)throw new Error("Value is out of range");this.write(new Uint8Array([t]))},this.writeUint16=t=>{if(t>65535||t<0)throw new Error("Value is out of range");this.write(new Uint8Array([t>>8,255&t]))},this.writeUint32=t=>{if(t>4294967295||t<0)throw new Error("Value is out of range");this.write(new Uint8Array([t>>24,t>>16&255,t>>8&255,255&t]))},this.writeInt8=t=>{if(t>127||t<-128)throw new Error("Value is out of range");this.write(new Uint8Array([t]))},this.writeInt16=t=>{if(t>32767||t<-32768)throw new Error("Value is out of range");this.write(new Uint8Array([t>>8,255&t]))},this.writeInt32=t=>{if(t>2147483647||t<-2147483648)throw new Error("Value is out of range");this.write(new Uint8Array([t>>24,t>>16&255,t>>8&255,255&t]))},this.writeBool=t=>{this.writeUint8(t?1:0)},this.writePackedBools=t=>{if(t.length>4)throw new Error("Each packed bool can only represent 4 or fewer values");let i=0;for(const r of t)i<<=1,r&&(i|=1);this.writeUint8(i)},this.writeString=t=>{const i=(new TextEncoder).encode(t),r=i.length;this.writeUint16(r),this.write(i)},this.writeSizedString=t=>{const i=(new TextEncoder).encode(t);this.write(i)},this.readUint8=()=>{this.start_reading();const t=new Uint8Array(1);return t.set(this.inner.subarray(this.pos,this.pos+1)),this.pos+=1,t[0]},this.readUint16=()=>{this.start_reading();const t=new Uint8Array(2);return t.set(this.inner.subarray(this.pos,this.pos+2)),this.pos+=2,t[0]<<8|t[1]},this.readUint32=()=>{this.start_reading();const t=new Uint8Array(4);return t.set(this.inner.subarray(this.pos,this.pos+4)),this.pos+=4,(t[0]<<24|t[1]<<16|t[2]<<8|t[3])>>>0},this.readInt8=()=>{this.start_reading();const t=new Uint8Array(1);t.set(this.inner.subarray(this.pos,this.pos+1)),this.pos+=1;const i=t[0];return 128&t[0]?-i:i},this.readInt16=()=>{this.start_reading();const t=new Uint8Array(2);t.set(this.inner.subarray(this.pos,this.pos+2)),this.pos+=2;const i=t[0]<<8|t[1];return 128&t[0]?-i:i},this.readInt32=()=>{this.start_reading();const t=new Uint8Array(4);t.set(this.inner.subarray(this.pos,this.pos+4)),this.pos+=4;const i=(t[0]<<24|t[1]<<16|t[2]<<8|t[3])>>>0;return 128&t[0]?-i:i},this.readBool=()=>{this.start_reading();const t=new Uint8Array(1);return t.set(this.inner.subarray(this.pos,this.pos+1)),this.pos+=1,1===t[0]},this.readPackedBools=()=>[],this.readString=()=>{this.start_reading();const t=this.readUint16(),i=this.inner.subarray(this.pos,this.pos+t),r=new TextDecoder("utf-8").decode(i);return this.pos+=t,r},this.readSizedString=t=>{this.start_reading();const i=this.inner.subarray(this.pos,this.pos+t),r=new TextDecoder("utf-8").decode(i);return this.pos+=t,r},this.readStringRemaining=()=>{this.start_reading();const t=this.inner.subarray(this.pos,this.inner.length),i=new TextDecoder("utf-8").decode(t);return this.pos=this.inner.length,i},this.serialize=t=>{},this.serializeNumber=t=>{},this.serializeString=t=>{},this.serializeBoolean=t=>{},this.inner=new Uint8Array(t),this.pos=0,this.reading=!1,this.capacity=1024}write(t){if(t.length>this.capacity||this.inner.length+t.length>this.capacity)throw new Error("Bufferfish is full");this.reading=!1;const i=new Uint8Array(this.inner.length+t.length);i.set(this.inner,0),i.set(t,this.inner.length),this.inner=i;const r=t.length;return this.pos+=r,r}start_reading(){this.reading||(this.pos=0,this.reading=!0)}set_max_capacity(t){if(t<1)throw new Error("Max capacity must be at least 1 byte");this.capacity=t}},Object.defineProperties(t,{__esModule:{value:!0},[Symbol.toStringTag]:{value:"Module"}})}));

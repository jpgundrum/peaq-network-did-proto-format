//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: did_document_format.proto

package io.peaq.did;

@kotlin.jvm.JvmName("-initializesignature")
public inline fun signature(block: io.peaq.did.SignatureKt.Dsl.() -> kotlin.Unit): io.peaq.did.Signature =
  io.peaq.did.SignatureKt.Dsl._create(io.peaq.did.Signature.newBuilder()).apply { block() }._build()
public object SignatureKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: io.peaq.did.Signature.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: io.peaq.did.Signature.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): io.peaq.did.Signature = _builder.build()

    /**
     * <code>.document.VerificationType type = 1;</code>
     */
    public var type: io.peaq.did.VerificationType
      @JvmName("getType")
      get() = _builder.getType()
      @JvmName("setType")
      set(value) {
        _builder.setType(value)
      }
    /**
     * <code>.document.VerificationType type = 1;</code>
     */
    public fun clearType() {
      _builder.clearType()
    }

    /**
     * <code>string issuer = 2;</code>
     */
    public var issuer: kotlin.String
      @JvmName("getIssuer")
      get() = _builder.getIssuer()
      @JvmName("setIssuer")
      set(value) {
        _builder.setIssuer(value)
      }
    /**
     * <code>string issuer = 2;</code>
     */
    public fun clearIssuer() {
      _builder.clearIssuer()
    }

    /**
     * <code>string hash = 3;</code>
     */
    public var hash: kotlin.String
      @JvmName("getHash")
      get() = _builder.getHash()
      @JvmName("setHash")
      set(value) {
        _builder.setHash(value)
      }
    /**
     * <code>string hash = 3;</code>
     */
    public fun clearHash() {
      _builder.clearHash()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun io.peaq.did.Signature.copy(block: io.peaq.did.SignatureKt.Dsl.() -> kotlin.Unit): io.peaq.did.Signature =
  io.peaq.did.SignatureKt.Dsl._create(this.toBuilder()).apply { block() }._build()


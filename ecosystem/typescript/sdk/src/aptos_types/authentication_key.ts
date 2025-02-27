// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

import sha3 from "js-sha3";
import { HexString } from "../hex_string";
import { Bytes } from "../bcs";
import { MultiEd25519PublicKey } from "./multi_ed25519";

const { sha3_256: sha3Hash } = sha3;

/**
 * Each account stores an authentication key. Authentication key enables account owners to rotate
 * their private key(s) associated with the account without changing the address that hosts their account.
 * @see {@link * https://aptos.dev/basics/basics-accounts | Account Basics}
 *
 * Account addresses can be derived from AuthenticationKey
 */
export class AuthenticationKey {
  static readonly LENGTH: number = 32;

  static readonly MULTI_ED25519_SCHEME: number = 1;

  readonly bytes: Bytes;

  constructor(bytes: Bytes) {
    if (bytes.length !== AuthenticationKey.LENGTH) {
      throw new Error("Expected a byte array of length 32");
    }
    this.bytes = bytes;
  }

  /**
   * Converts a K-of-N MultiEd25519PublicKey to AuthenticationKey with:
   * `auth_key = sha3-256(p_1 | … | p_n | K | 0x01)`. `K` represents the K-of-N required for
   * authenticating the transaction. `0x01` is the 1-byte scheme for multisig.
   */
  static fromMultiEd25519PublicKey(publicKey: MultiEd25519PublicKey): AuthenticationKey {
    const pubKeyBytes = publicKey.toBytes();

    const bytes = new Uint8Array(pubKeyBytes.length + 1);
    bytes.set(pubKeyBytes);
    bytes.set([AuthenticationKey.MULTI_ED25519_SCHEME], pubKeyBytes.length);

    const hash = sha3Hash.create();
    hash.update(bytes);

    return new AuthenticationKey(new Uint8Array(hash.arrayBuffer()));
  }

  /**
   * Derives an account address from AuthenticationKey. Since current AccountAddress is 32 bytes,
   * AuthenticationKey bytes are directly translated to AccountAddress.
   */
  derivedAddress(): HexString {
    return HexString.fromUint8Array(this.bytes);
  }
}

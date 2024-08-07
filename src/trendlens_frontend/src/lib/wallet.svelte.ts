import type { ActorSubclass, Identity } from "@dfinity/agent";
import type { _SERVICE } from "../../../declarations/trendlens_backend/trendlens_backend.did";
import { connect } from "./canisters";

interface IWallet {
  get connected(): boolean;
  connect(): Promise<void>;
  disconnect(): void;
}

class Wallet implements IWallet {
  private m_connected = $state(false);
  private m_actor = $state<ActorSubclass<_SERVICE> | null>(null);
  private m_identity = $state<Identity | null>(null);

  constructor() {}

  get connected() {
    return this.m_connected;
  }

  get actor() {
    return this.m_actor;
  }

  get identity() {
    return this.m_identity;
  }

  public async connect() {
    if (!this.m_connected) {
      const { actor, identity } = await connect();
      this.m_connected = true;
      this.m_actor = actor;
      this.m_identity = identity;
    }
  }

  public async disconnect() {
    this.m_connected = false;
    this.m_actor = null;
    this.m_identity = null;
  }
}

export const wallet = new Wallet();

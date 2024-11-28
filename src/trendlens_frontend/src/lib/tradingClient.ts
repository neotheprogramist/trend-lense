// Types for API requests and responses
export interface StartRequest {
  initial_balance: number;
  start: string; // ISO date string
  end: string; // ISO date string
  interval: "1m" | "1d";
}

export interface StepRequest {
  action: 0 | 1 | 2; // 0: Hold, 1: Buy, 2: Sell
}

export interface ResetRequest {
  initial_balance: number;
  start: string; // ISO date string
  end: string; // ISO date string
  interval: "1m" | "1d";
  indicators: string[];
}

export interface LastObservation {
  observation: Observation;
  observation_timestamp: string;
}

export interface Observation {
  prices: number[];
  portfolio: number[];
}

export interface StartResponse {
  token: string;
  observation: Observation;
}

export interface StepResponse {
  observation: Observation;
  reward: number;
  done: boolean;
  info: Record<string, any>;
}

// Add these interfaces
export interface GetActionsRequest {
  start: string;  // YYYY-MM-DD
  end: string;    // YYYY-MM-DD
  pair: string;   // e.g. "BTC-USD"
  interval: "1d"; // Currently only "1d" is supported
}

export interface Action {
  action: number;
  timestamp: string;
  reward: number;
  observation: Observation;
}

export interface GetActionsResponse {
  actions: Action[];
  start: string;
  end: string;
}

// Trading API Client
export class TradingClient {
  private baseUrl: string;
  private sessionToken: string | null = null;

  constructor(baseUrl: string = "http://localhost:5000") {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    method: "GET" | "POST",
    body?: any,
    useToken: boolean = true,
  ): Promise<T> {
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };

    if (useToken && this.sessionToken) {
      headers["X-Session-Token"] = this.sessionToken;
    }

    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || "API request failed");
    }

    return response.json();
  }

  async startSession(params: StartRequest): Promise<StartResponse> {
    const response = await this.request<StartResponse>(
      "/start",
      "POST",
      params,
      false,
    );
    this.sessionToken = response.token;
    return response;
  }

  async step(action: 0 | 1 | 2): Promise<StepResponse> {
    return this.request<StepResponse>("/step", "POST", { action });
  }

  async reset(params: ResetRequest): Promise<{ message: string }> {
    return this.request<{ message: string }>("/reset", "POST", params);
  }

  async getActions(params: GetActionsRequest): Promise<GetActionsResponse> {
    return this.request<GetActionsResponse>("/actions", "POST", params);
  }
}

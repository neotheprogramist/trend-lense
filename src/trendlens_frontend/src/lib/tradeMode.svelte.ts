class TradingMode {
  private v = $state<"single" | "multiple">("single");

  public switch() {
    if (this.v == "multiple") {
      this.v = "single";
    } else {
      this.v = "multiple";
    }
  }

  get value() {
    return this.v;
  }
}

export let tradingMode = new TradingMode();

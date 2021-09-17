class TabLabel extends HTMLElement {
  set active(value: boolean) {
    if (value) {
      this.classList.add("active");
    } else {
      this.classList.remove("active");
    }
  }

  get tabId(): string {
    return this.getAttribute("tabId");
  }
}

customElements.define("tab-label", TabLabel);

class TabPanel extends HTMLElement {
  get tabId(): string | null {
    return this.getAttribute("tabId");
  }

  connectedCallback() {
    this.hidden = true;
  }
}

customElements.define("tab-panel", TabPanel);

class TabsControler extends HTMLElement {
  private _onClick(event: MouseEvent) {
    const target = event.target as HTMLElement;

    if (target.tagName.toLowerCase() === "tab-label") {
      const tabLabel = target as TabLabel;
      if (tabLabel.tabId) {
        this.setAttribute("selectedId", tabLabel.tabId);
      }
    }
  }

  connectedCallback() {
    this.addEventListener("click", this._onClick);
  }

  disconnectedCallback() {
    this.removeEventListener("click", this._onClick);
  }

  static get observedAttributes() {
    return ["selectedid"];
  }

  attributeChangedCallback() {
    this.querySelectorAll("tab-label").forEach(
      (label) => ((label as TabLabel).active = false)
    );
    this.querySelectorAll("tab-panel").forEach(
      (label) => ((label as HTMLElement).hidden = true)
    );

    let tabId = this.getAttribute("selectedId");
    if (tabId) {
      const lable = this.querySelector(`tab-label[tabId=${tabId}]`) as TabLabel;
      const panel = this.querySelector(`tab-panel[tabId=${tabId}]`) as TabPanel;

      if (lable && panel) {
        lable.active = true;
        panel.hidden = false;
      }
    }
  }
}

customElements.define("tabs-controler", TabsControler);

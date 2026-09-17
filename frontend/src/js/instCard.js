class InstCard extends HTMLElement {
     connectedCallback() {
          const name = this.getAttribute('name');
          const src = this.getAttribute('src') || '/assets/images/icon.png';
          const alt = this.getAttribute('alt');
          this.innerHTML = `
               <button data-name="${name}" class="instance-card">
                    <span class="icon"><img src="${src}" alt="${alt}" width="30px"></span>
                    <span class="header"><h3>${name}</h3></span>
                    <span class="dots"><img src="/assets/images/three-dots.svg" alt="Dropdown" width="16px"></span>
               </button>
          `;
     }
}
customElements.define('inst-card', InstCard);
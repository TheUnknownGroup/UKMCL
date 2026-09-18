class InstCard extends HTMLElement {
     connectedCallback() {
          const name = this.getAttribute('name');
          const src = this.getAttribute('src') || '/assets/images/icon.png';
          const alt = this.getAttribute('alt');
          this.innerHTML = `
               <button data-name="${name}" class="instance-card flex rounded-lg bg-[rgba(38,66,16,0.95)] border-2 border-[#3f6e1a] w-full hover:shadow-[0_0_10px_rgba(0,0,0,0.4)] transition duration-300 ease-in-out p-[10px_0_10px_0] ml-1.25 cursor-pointer">
                    <span class="pl-2.25"><img src="${src}" alt="${alt}" width="30px"></span>
                    <span class="flex items-center"><h3 class="text-[20px] whitespace-nowrap text-ellipsis text-[#d4dacf] pl-2.25 select-none">${name}</h3></span>
                    <span class="flex ml-auto pr-1.25"><img src="/assets/images/three-dots.svg" alt="Dropdown" width="16px"></span>
               </button>
          `;
     }
}
customElements.define('inst-card', InstCard);
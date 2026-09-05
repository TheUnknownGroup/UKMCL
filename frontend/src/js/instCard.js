class InstCard extends HTMLElement {
     connectedCallback() {
          const name = this.getAttribute('name');
          this.innerHTML = `
               <div class="instance-card">
                    <h3>${name}</h3>
                    <div class="card-act">
                         <button class="launch-btn btn2" data-name="${name}"><img src="/assets/images/play.svg" alt="Launch"></button>
                         <button class="delete-btn btn2" data-name="${name}"><img src="/assets/images/trash.svg" alt="Delete"></button>
                         <button class="edit-btn btn2" data-name="${name}"><img src="/assets/images/tools.svg" alt="Edit"></button>
                    </div>
               </div>
          `;
     }
}
customElements.define('inst-card', InstCard);
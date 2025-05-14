const backup_el = `<a id="[id]-name">[name]</a><br>
        <a id="[id]-path">[path]</a><br>
        <a id="[id]-url">[url]</a><br>
        <button id="backup" backup_id="[id]">Back up NOW</button>
        <a id="[id]-time">Nan</a>
         <a id="[id]-size">[size]</a>
         <button id="remove_backup" backup_id="[id]">DELETE</button>
      `;
const task_el = `
      <a id="[id]-name">[name]</a><br>
      <a id="[id]-downloaded">[downloaded]</a>/<a id="[id]-size">[size]</a>
      <br>
      <progress max="100" value="[progress]" id="[id]-progress"></progress><a>[progress]%</a>  
      <a id="[id]-speed">[speed]</a>
    `;

export { backup_el, task_el };

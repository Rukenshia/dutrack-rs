(async () => {
    if (typeof window.STAMPS === 'undefined') {
        return;
    }

    async function getStamps() {
        const arr = [];

        for (const stamp of window.STAMPS.today) {
            arr.push(JSON.parse(await http.get(`/api/v1/stamps/${stamp}`)));
        }

        return arr;
    }

    const sel = document.getElementById('workday_stamps');
    const stamps = await getStamps();

    let lastStamp = null;

    stamps.forEach(stamp => {
        const div = document.createElement('div');
        
        div.innerHTML = `${stamp.event === 'enter' ? 'started' : 'stopped'} working ${moment(stamp.time).fromNow()}`;

        if (lastStamp) {
            if (lastStamp.event === stamp.event) {
                // ignore
                return;
            }

            if (lastStamp.event === 'enter' && stamp.event === 'exit') {
                div.innerHTML += `(worked for ${moment(stamp.time).from(moment(lastStamp.time), true)})`;
            }
        }

        div.innerHTML += '<br />';

        lastStamp = stamp;
        sel.appendChild(div);
    });
})();
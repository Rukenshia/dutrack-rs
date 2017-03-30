Vue.component('event', {
  props: ['type', 'from', 'until', 'now'],
  template: `
    <article class="media">
      <figure class="media-left">
        <span class="icon">
          <i class="fa" :class="icon"></i>
        </span>
      </figure>
      <div class="media-content">
        <div class="content">
          <p>
            <strong>{{ since }}</strong><small>&nbsp{{from.tz('Europe/Berlin').format('HH:mm')}}<template v-if="from !== until && until.diff(from) > 60000"> until {{until.tz('Europe/Berlin').format('HH:mm')}}</template></small>
            <br>
            {{ eventText }}
            <br>
          </p>
        </div>
      </div>
    </article>
    `,
  data() {
    return {};
  },
  computed: {
    since() {
      return this.now.to(this.from);
    },
    eventText() {
      return {
        start_work: 'Started working',
        stop_work: 'Stopped working',
        tiny_break: 'Took a very short break',
        short_break: `Took a break for ${moment.duration(this.until.diff(this.from)).humanize()}`,
        long_break: `Took a break for ${moment.duration(this.until.diff(this.from)).humanize()}`,
      }[this.type];
    },
    icon() {
      return {
        start_work: 'fa-briefcase',
        stop_work: 'fa-child',
        tiny_break: 'fa-bath',
        short_break: 'fa-coffee',
        long_break: 'fa-bed',
      }[this.type];
    },
  }
});
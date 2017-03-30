Vue.component('day-summary', {
  template: `
    <div class="day-summary">
      <h3 class="title" v-if="daysAgo === 0">Today</h3>
      <h3 class="title" v-else-if="daysAgo === 1">Yesterday</h3>
      <h3 class="title" v-else>{{ date }}</h3>
      <p class="subtitle" v-if="stamps.length > 0">time worked: {{ timeWorked.hours }}:{{ timeWorked.minutes }}</p>
      <p class="subtitle" v-else><span class="tag is-normal">no records</span></p>
      <span v-if="notCheckedOut" class="tag is-danger">not checked out</span>
    </div>
  `,
  props: {
    date: String,
    stamps: Array,
    now: {
      required: false,
    },
  },
  data() {
    return {
      daysAgo: 0,
      notCheckedOut: false,
    };
  },
  mounted() {
    this.daysAgo = moment().startOf('day').diff(moment(this.date).startOf('day'), 'days');
  },
  computed: {
    timeWorked() {
      if (!this.stamps) {
        return {
          hours: '00',
          minutes: '00',
        };
      }

      let dur = moment.duration(0);
      let enter = null;
      this.stamps.forEach(stamp => {
        const m = moment(stamp.time);
        if (enter && stamp.event === 'exit') {
          dur.add(m.diff(enter));
          enter = null;
        }

        if (stamp.event === 'enter') {
          enter = m;
        }
      });

      if (enter) {
        this.notCheckedOut = true;

        if (this.daysAgo === 0) {
          dur.add(this.now.diff(enter));
        }
      } else {
        this.notCheckedOut = false;
      }

      return {
        hours: `0${dur.hours()}`.slice(-2),
        minutes: `0${dur.minutes()}`.slice(-2),
      };
    }
  },
  methods: {
    timeSince(time) {
      return moment(time).from();
    }
  }
});
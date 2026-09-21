use std::time::Duration;

pub enum DynamicDurationKind {
    Increasing,
    Decreasing,
}

pub struct DynamicDuration {
    initial: Duration,
    limit: Duration,
    margin: Duration,
    /// Change per second
    rate: Duration,
    kind: DynamicDurationKind,
}

impl DynamicDuration {
    /// `margin` is the duration before incrementing starts
    /// `rate` is the change per second
    pub fn new(initial: Duration, limit: Duration, margin: Duration, rate: Duration, kind: DynamicDurationKind) -> Self {
        Self {
            initial,
            limit,
            margin,
            rate,
            kind,
        }
    }

    #[inline]
    pub fn get_duration(&self, elapsed: Duration) -> Duration {
        if self.margin >= elapsed {
            return self.initial;
        }

        let mut duration = self.initial;
        let seconds = (elapsed - self.margin).as_secs() as u32;

        match self.kind {
            DynamicDurationKind::Increasing => {
                duration += self.rate * seconds;

                if duration > self.limit {
                    duration = self.limit;
                }
            },
            DynamicDurationKind::Decreasing => {
                duration -= self.rate * seconds;

                if duration < self.margin {
                    duration = self.margin;
                }
            },
        }

        duration
    }
}

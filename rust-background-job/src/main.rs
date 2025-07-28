use job_scheduler::{ Job, JobScheduler };

fn main() {
    let mut scheduler = JobScheduler::new();
/***
 sec   min   hour   day of month   month   day of week   year
 *     *     *      *              *       *             *
***/
    scheduler.add(Job::new("3 * * * * * *".parse().unwrap(), || {
        println!("Sending email to existing Subscribes...!");
    }));
    loop {
        scheduler.tick();
    }
}

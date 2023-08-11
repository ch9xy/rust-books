

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}


#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}


Type Message = String;

struct GroundStation;

impl GroundStation {
    
    fn send(&self, mailbox: &mut MailBox, to: &CubeSat, msg:Message) {
        mailbox.post(to, msg);
    }

}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

fn main () {


}
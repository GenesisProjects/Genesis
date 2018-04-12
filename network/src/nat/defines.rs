pub mod stun {
    pub const STUN_IN_BUFFER_SIZE: usize   = 1024 * 1024;
    pub const STUN_OUT_BUFFER_SIZE: usize  = 1024 * 1024;

    pub const STUN_CLASS_MASK: u16 = 0x0110u16;

    /// STUN Message Type
    /// [[https://www.3cx.com/blog/voip-howto/stun-details/]]
    ///0x0001 : Binding Request
    ///0x0101 : Binding Response
    ///0x0111 : Binding Error Response
    ///0x0002 : Shared Secret Request
    ///0x0102 : Shared Secret Response
    ///0x0112 : Shared Secret Error Response
    pub enum STUNMessageType {
        BindingRequest,
        BindingResponse,
        BindingErrorResponse,
        SharedSecretRequest,
        SharedSecretResponse,
        SharedSecretErrorResponse
    }

    impl STUNMessageType {
        pub fn raw_value(&self) -> u16 {
            match self {
                &STUNMessageType::BindingRequest             => 0x0001u16,
                &STUNMessageType::BindingResponse            => 0x0101u16,
                &STUNMessageType::BindingErrorResponse       => 0x0111u16,
                &STUNMessageType::SharedSecretRequest        => 0x0002u16,
                &STUNMessageType::SharedSecretResponse       => 0x0102u16,
                &STUNMessageType::SharedSecretErrorResponse  => 0x0112u16
            }
        }
    }

    pub enum STUNClassType {
        Request,
        Indication,
        SuccessResp,
        ErrResponse
    }
}



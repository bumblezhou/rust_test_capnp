@0x95a82331f21ae640;

interface HelloWorld {
    struct HelloRequest {
        name @0 :Text;
    }

    struct HelloReply {
        message @0 :Text;
    }

    sayHello @0 (request: HelloRequest) -> (reply: HelloReply);
    whoAmI @1 () -> (reply: HelloReply);
    multuply @2 (a: Int16, b: Int16) -> (result: Int16);
    isOdd @3 (a: Int16) -> (isOdd: Bool);
    factorial @4(a: UInt16) -> (fact: UInt64);
}
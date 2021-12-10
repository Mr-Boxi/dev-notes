pragma solidity ^0.4.0;

// 简单的拍卖
contract SimpleAuction {
    // 拍卖的参数
    address public beneficiary;
    // 时间戳
    uint public auctionEnd;

    // 拍卖的当前状态
    address public highestBidder;
    uint public highestBid;

    // 可以取回之前的出价
    mapping(address => uint) pendingReturns;

    // 拍卖结束后设置为 true, 将禁止所有的变更
    bool ended;

    // 变更触发的事件
    event HighestBidIncreased(address bidder, uint amount);
    event AuctionEended(address winner, uint amount);

    /// 以受益者地址 `_beneficiary` 的名义，
    /// 创建一个简单的拍卖，拍卖时间为 `_biddingTime` 秒。
    constructor(
        uint _biddingTime,
        address _beneficiary
    )public {
        beneficiary = _beneficiary;
        auctionEnd = now + _biddingTime;
    }

    function bid() public payable {
        require(
            now <= auctionEnd,
            "Auction already ended"
        );

        require(
            msg.value > highestBid,
            "There already is a higher bid"
        );

    }
}

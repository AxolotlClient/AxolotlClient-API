import { Router } from "express";
import { userManager } from "../../..";
import { db } from "../../../../..";
import { User } from "../../../../database/entities/user";
const router = Router();

/**
 * Get known information about the user with the provided uuid.
 */
router.get("/:uuid", async (req, res) => {
  const uuid = req.params.uuid;
  const user = await db.getEntityManager().findOne(User, { uuid });
  if (!user) {
    res.status(404).json({ message: "User not found." });
    return;
  }
  res.json({
    uuid: user.uuid,
    friends: user.friends,
    online: userManager.isOnline(user.uuid),
  });
});

/**
 * Create/Update the status of a user.
 */

router.post("/:uuid", async (req, res) => {

    const uuid = req.params.uuid;
    const online = req.body.online;
    
    const user = await db.getEntityManager().findOne(User, { uuid });
    if (!user) {
        res.status(404).json({ message: `User id ${uuid} not found.` });
        return;
    }
    
    userManager.setStatus(uuid, online);
    
    res.status(200).json({ message: "OK" });    
    
});


/**
 * Get a user's friends.
 */

router.get("/:uuid/friends", async (req, res) => {

    const uuid = req.params.uuid;
    
    const user = await db.getEntityManager().findOne(User, { uuid });
    if (!user) {
        res.status(404).json({ message: `User id ${uuid} not found.` });
        return;
    }

    const friends = await db.getEntityManager().find(User, { uuid: user.friends });
    
    res.json(friends.map((friend) => {
        return {
            uuid: friend.uuid,
            online: userManager.isOnline(friend.uuid)
        }
    }));

})

router.post("/:uuid/friends", async (req, res) => {

    const uuid = req.params.uuid;
    const friendUuid = req.body.friend;

    const user = await db.getEntityManager().findOne(User, { uuid });
    if (!user) {
        res.status(404).json({ message: `User id ${uuid} not found.` });
        return;
    }

    const friend = await db.getEntityManager().findOne(User, { uuid: friendUuid });
    if (!friend) {
        res.status(404).json({ message: `User id ${friendUuid} not found.` });
        return;
    }

    user.friends.push(friend.uuid);

    await db.getEntityManager().persistAndFlush(user);
    res.status(200).json({ message: "OK" });
})

export default router;

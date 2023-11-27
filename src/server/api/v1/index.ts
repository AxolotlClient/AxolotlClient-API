import { Router } from 'express';
import { userManager } from '../..';

import userRouter from './routes/user'

const router = Router();

router.use('/user', userRouter);

/**
 * /count
 * Get the count of known and currently online users.
 */
router.get('/count', async (req, res) => {
    const count = await userManager.getCount();
    res.json(count);
})



export default router;
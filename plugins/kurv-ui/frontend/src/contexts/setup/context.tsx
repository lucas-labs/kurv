import { type Dispatch, type SetStateAction, createContext } from 'react';
import type { SetupStatus, SetupStatusResponse } from '@/hooks/use-api/types';

/** Context for providing the setup status of the system */
export const SetupStatusContext = createContext<
    SetupStatusResponse & {
        fetchStatus?: () => Promise<void>;
        setStatus?: Dispatch<SetStateAction<SetupStatus>>;
        error?: string;
    }
>({ status: 'loading' });

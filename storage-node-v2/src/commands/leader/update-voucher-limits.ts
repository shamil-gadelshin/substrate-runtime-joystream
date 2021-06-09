import ApiCommandBase from '../../command-base/ApiCommandBase'
import { updateStorageBucketsVoucherMaxLimits } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'

export default class LeaderUpdateVoucherLimits extends ApiCommandBase {
  static description =
    'Updates VoucherMaxObjectsSizeLimit and VoucherMaxObjectsNumberLimit the Joystream node storage.'

  static flags = {
    objects: flags.integer({
      char: 'o',
      required: true,
      description: `New 'max voucher object number limit' value`,
    }),
    size: flags.integer({
      char: 's',
      required: true,
      description: `New 'max voucher object size limit' value`,
    }),
    ...ApiCommandBase.keyflags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateVoucherLimits)

    this.log('Update "Storage buckets per bag" number limit....')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const objectsLimit = flags.objects ?? 0
    const sizeLimit = flags.size ?? 0

    await updateStorageBucketsVoucherMaxLimits(account, sizeLimit, objectsLimit)
  }
}
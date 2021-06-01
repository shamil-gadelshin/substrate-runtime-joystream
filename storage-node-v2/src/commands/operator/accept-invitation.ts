import { flags } from '@oclif/command'
import { acceptStorageBucketInvitation } from '../../services/extrinsics'
import ApiCommandBase from '../../command-base/ApiCommandBase'

export default class OperatorAcceptInvitation extends ApiCommandBase {
  static description = 'Accept pending storage bucket invitation.'

  static flags = {
    worker: flags.integer({
      char: 'w',
      required: true, // TODO: for dev
      description: 'Storage operator worker ID',
    }),
    bucket: flags.integer({
      char: 'b',
      required: true,
      description: 'Storage bucket ID',
    }),
    ...ApiCommandBase.keyflags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(OperatorAcceptInvitation)

    const worker = flags.worker ?? 0 // TODO: don't require on dev???
    const bucket = flags.bucket ?? 0

    this.log('Accepting pending storage bucket invitation...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)

    await acceptStorageBucketInvitation(account, worker, bucket)
  }
}